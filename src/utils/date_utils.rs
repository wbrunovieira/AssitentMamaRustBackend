use crate::models::date::DateInfo;
use chrono::{Datelike, Local};

pub fn get_current_date_info() -> DateInfo {
    let now = Local::now();

    let weekday = match now.weekday() {
        chrono::Weekday::Mon => "Segunda-feira",
        chrono::Weekday::Tue => "Terça-feira",
        chrono::Weekday::Wed => "Quarta-feira",
        chrono::Weekday::Thu => "Quinta-feira",
        chrono::Weekday::Fri => "Sexta-feira",
        chrono::Weekday::Sat => "Sábado",
        chrono::Weekday::Sun => "Domingo",
    };

    DateInfo {
        day: now.day(),
        month: now.month(),
         year: now.year(),
        weekday: weekday.to_string(),
    }
}

pub fn get_month_name(month: u32) -> String {
    match month {
        1 => "Janeiro".to_string(),
        2 => "Fevereiro".to_string(),
        3 => "Março".to_string(),
        4 => "Abril".to_string(),
        5 => "Maio".to_string(),
        6 => "Junho".to_string(),
        7 => "Julho".to_string(),
        8 => "Agosto".to_string(),
        9 => "Setembro".to_string(),
        10 => "Outubro".to_string(),
        11 => "Novembro".to_string(),
        12 => "Dezembro".to_string(),
        _ => "Mês desconhecido".to_string(),
    }
}
