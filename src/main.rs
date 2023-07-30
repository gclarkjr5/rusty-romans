use leptos::*;

use rusty_romans::output::Output;
use rusty_romans::error::InputError;


#[component]
fn App(cx: Scope) -> impl IntoView {
    let (read_value, set_value) = create_signal(cx, Ok(0.to_string()));

    let validated_value = move |event| {
        let ev = event_target_value(&event);

        let validated_event = match parse_value_to_integer(&ev) {
            Ok(int) => validate_integer_input(int),
            Err(_) => validate_roman_numeral_input(&ev),
        };

        set_value.set(validated_event)
    };

    

    let converted_value = move |_| {

        let mut output = Output::init();

        let res = match read_value.get() {
            Ok(i) => {
                match parse_value_to_integer(&i) {
                    Ok(i) => output.convert_integer(&i),
                    Err(_) => output.convert_roman_numeral(&read_value.get().unwrap())
                }
            },
            Err(e) => {
                output.output = Err(e);
                &mut output
            }
            
        };

        res.output.clone()
    };
    

    view! {
        cx,  
        "Type an Integer or a Roman Numeral: "
        <input type="text" on:input=validated_value/>          
        <ErrorBoundary
            fallback=|cx, errors| view! {
                cx,
                <div class="error" style="color: red">
                    <p>"Not a number! Errors: "</p>
                    <ul>
                        {
                            move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! {cx, <li>{e.to_string()}</li>})
                                .collect::<Vec<_>>()
                        }
                    </ul>
                </div>
            }
        >
            <p>"The Converted value is: " {converted_value}</p>
        </ErrorBoundary>
        
    }

}


fn main() {
    leptos::mount_to_body(|cx| view! { cx,  <App/> })
}

fn parse_value_to_integer(value: &str) -> Result<i64, std::num::ParseIntError> {

    value.parse::<i64>()

}

fn validate_integer_input(i: i64) -> Result<String, InputError> {
    // error conditions
    if i > 3999 {
        return  Err(InputError::TooBigError)
        // return an error
    } else if i < 1  {
        return Err(InputError::TooSmallError)
        // return an error
    }

    Ok(i.to_string())
}

fn validate_roman_numeral_input(roman_numeral: &str) -> Result<String, InputError> {
    let valid_roman_numerals: Vec<char> = "MDCLXVI".chars().collect();

    if roman_numeral.to_uppercase().chars().any(|c| !valid_roman_numerals.contains(&c)) {
        println!("this");
        return Err(InputError::ContainsNotRomanNumeral)
    }

    Ok(roman_numeral.to_string())
}

pub fn convert_integer(i: &i64) -> Result<String, InputError> {

    // # divide number to understand amount and remainder
    // # start with the highest roman numeral digit and work down
    // # keep a remainder reference as we progress
    let remainder = *i;

    // # if atleast 1 thousand
    let (thousand, remainder) = get_amount_and_remainder(remainder, 1000);
    // # next weed out the five_hundreds
    let (five_hundred, remainder) = get_amount_and_remainder(remainder, 500);
    // # now the one_hundreds
    let (one_hundred, remainder) = get_amount_and_remainder(remainder, 100);
    // # now 50s
    let (fifty, remainder) = get_amount_and_remainder(remainder, 50);
    // # now 10s
    let (ten, remainder) = get_amount_and_remainder(remainder, 10);
    // # now 5s
    let (five, remainder) = get_amount_and_remainder(remainder, 5);

    // # create concatenated strings of the individual roman numerals
    // # i.e ['MM', 'D', '', 'L', 'XX', 'V', 'III']
    let i_list = vec![
        ("M", thousand),
        ("D", five_hundred),
        ("C", one_hundred),
        ("L", fifty),
        ("X", ten),
        ("V", five),
        ("I", remainder),
    ];

    // convert the above list of tuples into a string of all roman numerals
    let mut converted_string: Vec<String> = vec![];
    for (roman_numeral, amt) in i_list {
        let res = create_roman_numeral_string(roman_numeral, amt);

        converted_string.push(res)
    }

    // # Rule: You cannot have more than 3 consecuitve of the same roman numeral
    // # i.e: 3 == III, but 4 == IV and NOT IIII
    // # another example: 8 == VIII, but 9 == IX and not VIIII
    // # this behavior seems to only happen if a digit is a 4 or a 9
    // # in the hundreds, tens, or ones digits
    // # the below is meant to handle this

    // # if one_hundred occurs 4x and five_hundred 1x or 0x -> CM or CD
    if (one_hundred == 4) & (five_hundred == 1) {
        converted_string[1] = "C".to_string();
        converted_string[2] = "M".to_string();
    }

    if (one_hundred == 4) & (five_hundred == 0) {
        converted_string[1] = "C".to_string();
        converted_string[2] = "D".to_string();
    }

    // # if ten occurs 4x and fifty 1x or 0x -> XC or XL
    if (ten == 4) & (fifty == 1) {
        converted_string[3] = "X".to_string();
        converted_string[4] = "C".to_string();
    }

    if (ten == 4) & (fifty == 0) {
        converted_string[3] = "X".to_string();
        converted_string[4] = "L".to_string();
    }

    // # if remainder occurs 4x and five 1x or 0x -> IX or IV
    if (remainder == 4) & (five == 1) {
        converted_string[5] = "I".to_string();
        converted_string[6] = "X".to_string();
    }

    if (remainder == 4) & (five == 0) {
        converted_string[5] = "I".to_string();
        converted_string[6] = "V".to_string();
    }

    // # print(converted_string)
    // let final_string = "".join(converted_string);
    let final_string = converted_string.join("");

    Ok(final_string)

    // self.output = Ok(final_string);

    // self
}

fn get_amount_and_remainder(mut remainder: i64, unit: i64) -> (i64, i64) {
    let mut amount = 0;
    if remainder / unit >= 1 {

        amount = remainder / unit;

        // # subtract how many thousands from the original number
        // # update the higher level reference remainder
        remainder -= unit * amount;
    }

    (amount, remainder)
}

fn create_roman_numeral_string(roman_numeral: &str, amount: i64) -> String {
    
    vec![roman_numeral; amount as usize].join("")

}
