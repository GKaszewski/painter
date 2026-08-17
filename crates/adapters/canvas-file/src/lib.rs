use std::fs;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use config::SnapshotConfig;
use domain::ports::CanvasPersistence;
use domain::{Color, DomainError};
use tracing::{error, info};

pub struct FileCanvasPersistence {
    directory: PathBuf,
    max_snapshots: usize,
}

impl FileCanvasPersistence {
    pub fn new(config: &SnapshotConfig) -> Result<Self, DomainError> {
        let directory = PathBuf::from(&config.directory);
        fs::create_dir_all(&directory).map_err(|err| {
            DomainError::Persistence(format!(
                "Failed to create snapshot directory '{}': {err}",
                directory.display()
            ))
        })?;

        Ok(Self {
            directory,
            max_snapshots: config.max_snapshots,
        })
    }

    fn snapshot_path(&self, index: usize) -> PathBuf {
        self.directory.join(format!("canvas_{index}.bin"))
    }

    fn latest_index_path(&self) -> PathBuf {
        self.directory.join("latest")
    }

    fn read_latest_index(&self) -> Option<usize> {
        let path = self.latest_index_path();
        fs::read_to_string(&path)
            .ok()
            .and_then(|content| content.trim().parse().ok())
    }

    fn write_latest_index(&self, index: usize) -> Result<(), DomainError> {
        let path = self.latest_index_path();
        fs::write(&path, index.to_string())
            .map_err(|err| DomainError::Persistence(format!("Failed to write latest index: {err}")))
    }

    fn rotate_and_cleanup(&self, new_index: usize) {
        if self.max_snapshots == 0 {
            return;
        }
        let oldest_to_keep = new_index.saturating_sub(self.max_snapshots - 1);
        for stale_index in 0..oldest_to_keep {
            let stale_path = self.snapshot_path(stale_index);
            if stale_path.exists()
                && let Err(err) = fs::remove_file(&stale_path)
            {
                error!(
                    "Failed to remove old snapshot '{}': {err}",
                    stale_path.display()
                );
            }
        }
    }
}

impl CanvasPersistence for FileCanvasPersistence {
    fn save(&self, pixels: &[Color]) -> Result<(), DomainError> {
        let next_index = self.read_latest_index().map(|i| i + 1).unwrap_or(0);
        let path = self.snapshot_path(next_index);

        write_pixels_to_file(&path, pixels)?;
        self.write_latest_index(next_index)?;
        self.rotate_and_cleanup(next_index);

        info!(
            "Saved canvas snapshot #{next_index} to '{}'",
            path.display()
        );
        Ok(())
    }

    fn load_latest(&self) -> Result<Option<Vec<Color>>, DomainError> {
        let Some(index) = self.read_latest_index() else {
            info!("No canvas snapshots found");
            return Ok(None);
        };

        let path = self.snapshot_path(index);
        if !path.exists() {
            info!("Snapshot file '{}' not found", path.display());
            return Ok(None);
        }

        let pixels = read_pixels_from_file(&path)?;
        info!(
            "Loaded canvas snapshot #{index} from '{}' ({} pixels)",
            path.display(),
            pixels.len()
        );
        Ok(Some(pixels))
    }
}

fn write_pixels_to_file(path: &Path, pixels: &[Color]) -> Result<(), DomainError> {
    let file = fs::File::create(path).map_err(|err| {
        DomainError::Persistence(format!(
            "Failed to create snapshot '{}': {err}",
            path.display()
        ))
    })?;
    let mut writer = BufWriter::new(file);
    for color in pixels {
        writer
            .write_all(&color.as_u32().to_ne_bytes())
            .map_err(|err| {
                DomainError::Persistence(format!(
                    "Failed to write snapshot '{}': {err}",
                    path.display()
                ))
            })?;
    }
    writer.flush().map_err(|err| {
        DomainError::Persistence(format!(
            "Failed to flush snapshot '{}': {err}",
            path.display()
        ))
    })?;
    Ok(())
}

fn read_pixels_from_file(path: &Path) -> Result<Vec<Color>, DomainError> {
    let bytes = fs::read(path).map_err(|err| {
        DomainError::Persistence(format!(
            "Failed to read snapshot '{}': {err}",
            path.display()
        ))
    })?;

    if bytes.len() % 4 != 0 {
        return Err(DomainError::Persistence(format!(
            "Snapshot '{}' has invalid size: {} bytes (not a multiple of 4)",
            path.display(),
            bytes.len()
        )));
    }

    let pixels = bytes
        .chunks_exact(4)
        .map(|chunk| Color::new(u32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])))
        .collect();

    Ok(pixels)
}
