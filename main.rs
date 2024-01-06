use std::{thread, time::Duration};

use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new(
        "http://localhost:9515", // this is the chromedriver server + port
        caps,
    )
    .await?;

    // go the binary game page
    driver
        .goto("https://learningcontent.cisco.com/games/binary/index.html")
        .await?;

    // finds the start button and clicks it to the start the game
    let elem = driver.find(By::ClassName("modal-body")).await?;

    let start_button = elem
        .find(By::Css("button[style='margin-bottom: 8px;']"))
        .await?;

    start_button.click().await?;

    loop {
        loop {
            // find the problem
            let problem_elem = driver
                .query(By::ClassName("slide-fade-enter-done"))
                .wait(Duration::from_secs(60), Duration::from_secs(1))
                .first()
                .await?;

            // find all the bits in the problem
            let bits = problem_elem
                .find(By::ClassName("bits"))
                .await?
                .find_all(By::ClassName("bit"))
                .await?;

            // get the digit text
            // there is probably a better way to do this lmfao but element.text() doesn't work
            let digits_elem = problem_elem
                .query(By::ClassName("digits"))
                .wait(Duration::from_secs(60), Duration::from_secs(1))
                .first()
                .await?;
            let html_tag_re = regex::Regex::new(r"<[^>]*>").unwrap();
            let binding = digits_elem.inner_html().await?;
            let digits_text = html_tag_re.replace_all(&binding, "");

            // figure out if the first problem is a conversion from binary to decimal or vice versa
            if digits_elem
                .class_name()
                .await?
                .unwrap()
                .contains("isProblem")
            {
                println!("Problem is of type Binary to Decimal");

                // get the num from the binary
                let mut num_str: String = "".to_owned();
                for bit in bits {
                    // get the bit text
                    let binding = bit.inner_html().await?;
                    let bit_text = html_tag_re.replace_all(&binding, "");

                    if bit_text == "1" {
                        num_str.push('1');
                    } else {
                        num_str.push('0');
                    }
                }

                // convert the binary to decimal
                let num = i32::from_str_radix(&num_str, 2).unwrap();

                println!("decimal: {}", num);

                // click on the element to bring up the calculator
                digits_elem.click().await?;

                // find the calculator and wait for it to load
                let calc_elem = driver
                    .query(By::ClassName("fade-enter-done"))
                    .wait(Duration::from_secs(60), Duration::from_secs(1))
                    .first()
                    .await?;

                // for each digit in the number, find the button and click it
                let num_buttons = calc_elem.find_all(By::ClassName("button")).await?;
                for digit in num.to_string().chars() {
                    // turn this into a match statement
                    match digit {
                        '0' => num_buttons.get(1).unwrap().click().await?,
                        '1' => num_buttons.get(3).unwrap().click().await?,
                        '2' => num_buttons.get(4).unwrap().click().await?,
                        '3' => num_buttons.get(5).unwrap().click().await?,
                        '4' => num_buttons.get(6).unwrap().click().await?,
                        '5' => num_buttons.get(7).unwrap().click().await?,
                        '6' => num_buttons.get(8).unwrap().click().await?,
                        '7' => num_buttons.get(9).unwrap().click().await?,
                        '8' => num_buttons.get(10).unwrap().click().await?,
                        '9' => num_buttons.get(11).unwrap().click().await?,
                        _ => (),
                    }
                }

                // click the go button
                num_buttons.get(2).unwrap().click().await?;
                thread::sleep(Duration::from_secs(1));
            } else {
                println!("Problem is of type Decimal to Binary");

                println!("decimal: {}", digits_text);

                // convert the decimal to binary
                // pad the binary with 0s to make it 8 bits
                let binary = format!("{:b}", digits_text.parse::<i32>().unwrap());
                let binary = format!("{:0>8}", binary);

                println!("binary: {}", binary);

                // for each bit, click it if it is a 1
                for (i, bit) in bits.iter().enumerate() {
                    // get the bit text
                    let binding = bit.inner_html().await?;
                    let bit_text = html_tag_re.replace_all(&binding, "");

                    // println!("bit: {}", bit_text);

                    if bit.attr("disabled").await? != None {
                        continue;
                    }

                    if binary.chars().nth(i).unwrap() != bit_text.chars().nth(0).unwrap() {
                        bit.click().await?;
                    }
                }
            }

            // get lines left and check if we won the level
            let game_stats = driver
                .query(By::ClassName("gameStats"))
                .wait(Duration::from_secs(60), Duration::from_millis(10))
                .first()
                .await?;

            let stats_elems = game_stats.find_all(By::ClassName("item")).await?;

            let lines_left_stats = stats_elems.get(2).unwrap();
            let lines_left_elems = lines_left_stats.find_all(By::Tag("span")).await?;
            let lines_left_elem = lines_left_elems.get(1).unwrap();
            let lines_left = lines_left_elem.inner_html().await?;

            println!("lines left: {}", lines_left);
            if lines_left == "0" {
                println!("level finished");
                break;
            }
        }

        // level finished so wait for the modal to pop up
        let modal = driver
            .query(By::ClassName("fade-enter-done"))
            .wait(Duration::from_secs(60), Duration::from_secs(1))
            .first()
            .await?;

        let modal_body = modal.find(By::ClassName("modal-body")).await?;

        // click the next level button
        let next_level_button = modal_body.find(By::Tag("button")).await?;

        next_level_button.click().await?;
    }
}
