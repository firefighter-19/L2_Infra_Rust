#[warn(clippy::all, clippy::pedantic)]
use rfd::FileDialog;
use std::env;
use std::fs;
use std::io::{Write, stdin, stdout};
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use screenshots::Screen;
use chrono::Utc;

use rdev::{Event, EventType, Key, grab};

const DEFAULT_DIR: &str = "screenshots";
const CONFIG_FILE: &str = ".screen_shooter.conf";

fn ask() -> bool {
    loop {
        print!("Вы хотите выбрать директорию вручную? [Y/n]: ");
        stdout().flush().unwrap();

        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();

        match s.trim() {
            "y" | "Y" => return true,
            "n" | "N" => return false,
            "" => return true,
            _ => println!("Введите Y или N."),
        }
    }
}

fn get_config_path() -> std::io::Result<PathBuf> {
    let current_dir = env::current_dir()?;
    Ok(current_dir.join(CONFIG_FILE))
}

fn read_config() -> Option<PathBuf> {
    let config_path = get_config_path().ok()?;
    let content = fs::read_to_string(&config_path).ok()?;
    let path_str = content.trim();
    if path_str.is_empty() {
        return None;
    }
    let path = PathBuf::from(path_str);
    if path.exists() && path.is_dir() {
        Some(path)
    } else {
        None
    }
}

fn save_config(path: impl AsRef<Path>) -> std::io::Result<()> {
    let config_path = get_config_path()?;
    fs::write(&config_path, path.as_ref().to_string_lossy().as_ref())?;
    Ok(())
}

fn pick_dir_manual() -> Option<PathBuf> {
    FileDialog::new().set_directory(".").pick_folder()
}

fn define_dir() -> std::io::Result<PathBuf> {
    let mut path_dir = env::current_dir()?;

    if let Some(saved_dir) = read_config() {
        println!(
            "Используем сохраненную директорию: {:?}",
            saved_dir.to_string_lossy()
        );
        path_dir = saved_dir;
    } else {
        if ask() {
            match pick_dir_manual() {
                Some(dir) => {
                    path_dir = dir;
                    println!("Вы выбрали: {:?}", path_dir.to_string_lossy());
                    if let Err(e) = save_config(&path_dir) {
                        eprintln!("Предупреждение: не удалось сохранить конфигурацию: {}", e);
                    }
                }
                None => {
                    println!(
                        "Пользователь отменил выбор директории, используем директорию по умолчанию"
                    );
                    path_dir.push(Path::new(DEFAULT_DIR));
                }
            }
        } else {
            println!("Используем директорию по умолчанию");
            path_dir.push(Path::new(DEFAULT_DIR));
        }
    }

    println!("Финальная директория: {:?}", path_dir.to_string_lossy());

    fs::create_dir_all(&path_dir)?;

    Ok(path_dir)
}

fn grab_screenshot(path_dir: Arc<PathBuf>) -> std::io::Result<()> {
    let handler_path = path_dir.clone();
    grab(move |e| callback(e, handler_path.clone())).map_err(|e| {
        eprintln!("Ошибка при захватке событий клавиатуры: {:?}", e);
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Ошибка при захватке скриншота: {:?}", e),
        )
    })
}

fn callback(event: Event, path_dir: Arc<PathBuf>) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::F1) => {
            let screenshot_path = path_dir.clone();
            thread::spawn(move || {
                if let Err(e) = take_screenshot(&screenshot_path) {
                    eprintln!("Ошибка при захватке скриншота: {}", e);
                }
            });
            None
        }
        _ => Some(event),
    }
}

fn take_screenshot(path_dir: &Path) -> std::io::Result<()> {
    let screens = match Screen::all() {
        Ok(screens) => screens,
        Err(e) => {
            eprintln!("Ошибка при получении списка экранов: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Не удалось получить список экранов: {:?}", e),
            ));
        }
    };
    
    for screen in screens {
        let image = match screen.capture() {
            Ok(image) => image,
            Err(e) => {
                eprintln!("Ошибка при захвате экрана: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Не удалось захватить экран: {:?}", e),
                ));
            }
        };

        let utc = Utc::now();
        let iso = utc.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let file_path = path_dir.join(format!("screenshot_{}.png", iso));

        match image.save(file_path.as_path()) {
            Ok(()) => println!("Скриншот сохранен: {}", file_path.display()),
            Err(e) => {
                eprintln!("Ошибка при сохранении скриншота: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Не удалось сохранить скриншот: {:?}", e),
                ));
            }
        }
    }
    Ok(())
}

fn main() {
    let path_dir = match define_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Ошибка при определении директории: {}", e);
            return;
        }
    };
    if let Err(e) = grab_screenshot(Arc::new(path_dir)) {
        println!("Ошибка при захватке скриншота: {}", e);
        return;
    }
}
