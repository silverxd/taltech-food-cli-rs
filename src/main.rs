pub mod constants;
pub mod serialization;

use crate::constants::{FOOD_URL, PREFERRED_DINERS};
use crate::serialization::Diners;
use colored::Colorize;
use reqwest::Error;
use std::env;

fn get_menu() -> Result<Diners, Error> {
    let food_request = reqwest::blocking::get(FOOD_URL)?;
    let result = food_request.json()?;
    Ok(result)
}

fn print_menu(menu: Diners) {
    menu.into_iter().for_each(|diner| {
        let mut diner_name = diner
            .diner
            .name
            .bold()
            .underline()
            .bright_yellow()
            .to_string();
        if let Some(settings) = diner.diner.settings
            && settings.offers_start.is_some()
            && settings.offers_end.is_some()
        {
            let open_times = format!(
                " {}-{}",
                settings.offers_start.unwrap(),
                settings.offers_end.unwrap()
            )
            .bright_green()
            .to_string();
            diner_name += &open_times;
        }
        println!("{}", diner_name);
        diner.meals.into_iter().for_each(|meal| {
            let meal_price = meal.price.unwrap_or("".to_string()).bright_blue();
            let meal = format!("- {}  {}", meal.description, meal_price);
            println!("{}", meal);
        });
        print!("\n\n");
    });
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let menu: Diners = match get_menu() {
        Ok(diners) => diners,
        Err(e) => {
            println!("Failed to get food info: {}", e);
            return Err(e);
        }
    };
    let selected_diners: Vec<String> = match args.len() {
        1 => PREFERRED_DINERS.iter().map(|&x| x.into()).collect(),
        _ => {
            let keywords: Vec<String> =
                args.into_iter().skip(1).map(|x| x.to_lowercase()).collect();
            PREFERRED_DINERS
                .iter()
                .filter(|diner| {
                    keywords
                        .iter()
                        .any(|keyword| diner.to_lowercase().contains(keyword))
                })
                .map(|&x| x.into())
                .collect()
        }
    };
    let mut menu: Diners = menu
        .into_iter()
        .filter(|x| selected_diners.contains(&x.diner.name))
        .collect();

    menu.sort_by(|a, b| {
        selected_diners
            .iter()
            .position(|p| p == &a.diner.name)
            .cmp(&selected_diners.iter().position(|p| p == &b.diner.name))
    });

    print_menu(menu);
    Ok(())
}
