use std::fmt;

#[derive (Clone, Default, Debug)]

/// Конфиг, в котором хранится информация о входной строчке
pub struct Config {
    /// Порт, в котором хранится число от 1 до 65535
    pub port: Option<u16>,
    /// Аргумент помощи, если *true* - выводим информацию об аргументах
    pub help: Option<bool>,
    /// Аргумент помощи, если *true* - записываем логи программы
    pub debug: Option<bool>,
}

#[derive(Debug)]
/// Перечесление ошибок
pub enum ParseError {
    MissingValue(String),
    InvalidValue(String, String),
    UnknownOption(String),
    AlreadySpecified(String),
    UnexpectedArgument(String),
}
/// Добавляем метод для отображения ошибок
impl fmt::Display for ParseError {
    /// Отображение ошибок
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::MissingValue(opt) => write!(f, "Missing value for option: {}", opt),
            ParseError::InvalidValue(opt, val) => write!(f, "Invalid value '{}' for option: {}", val, opt),
            ParseError::UnknownOption(opt) => write!(f, "Unknown option: {}", opt),
            ParseError::AlreadySpecified(opt) => write!(f, "Option '{}' already specified", opt),
            ParseError::UnexpectedArgument(arg) => write!(f, "Unexpected argument: {}", arg),
        }
    }
}

/// Класс ParamHandler, который реализует валидацию аргументов в входной строчке
pub struct ParamHandler {
    config: Config,
}

impl ParamHandler {
    /// Создаем новый ParamHandler
    pub fn new() -> Self {
        ParamHandler {
            config: Config::default(),
        }
    }
    /// Парсим аргументы из входно строки
    /// Изменяем конфиг, если нашли флаг
    /// params: &[String] - входная строка, разбытая на токены. Получается с помощью env::args().collect()
    /// Возвращяем или конфиг (в случае, когда входная строчка валидны), или ошибка (в противном случае)
    pub fn parse_args(&mut self, params: &[String]) -> Result<Config, ParseError> {
        let mut i = 1; // Пропускаем имя файла

        while i < params.len() {
            let arg = &params[i];

            if arg.starts_with('-') { // Каждый аргумент начинается с "-" 
                match arg.as_str() {
                    "--port" | "-port" => {
                        if i + 1 >= params.len() {
                            return Err(ParseError::MissingValue(arg.clone()));
                        }
                        self.parse_port(&params[i + 1])?;
                        i += 2;
                    }
                    // Для булевых значений нам не важно, сколько раз повторялся аргумент.
                    // Попробуй команду ls -l -l
                    "--help" | "-help" | "-h" => {
                        self.config.help = Some(true);
                        i += 1;
                    }
                    "--debug" | "-debug" | "-d" => {
                        self.config.debug = Some(true);
                        i += 1;
                    }
                    _ => return Err(ParseError::UnknownOption(arg.clone())),
                }
            } else {
                return Err(ParseError::UnexpectedArgument(arg.clone()));
            }
        }

        Ok(self.config.clone())
    }
    /// Метод, описывающий опработку порта
    /// Т.к после порта всегда должно идти число
    fn parse_port(&mut self, p: &str) -> Result<(), ParseError> {
        if self.config.port.is_some() {
            return Err(ParseError::AlreadySpecified("--port".to_string()));
        }

        match p.parse::<u16>() {
            Ok(number) => {
                if number == 0 {Err(ParseError::InvalidValue("--port".to_string(), p.to_string()))} else {
                    self.config.port = Some(number);
                    Ok(())
                }
            }
            Err(_) => Err(ParseError::InvalidValue("--port".to_string(), p.to_string())),
        }
    }
    /// Возвращает неизменяемую ссылку на конфиг
    pub fn get_config(&self) -> &Config {
        &self.config
    }
}    
