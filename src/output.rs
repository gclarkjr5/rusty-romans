use crate::error::InputError;

pub struct Output {
    pub output: Result<String, InputError>
}

impl Output {
    pub fn init() -> Self {
        Output { output: Ok("".to_string()) }
    }

    pub fn convert_roman_numeral(&mut self, roman_numeral: &str) -> &mut Self {
        // """Receives an input like a roman numeral or an arabic
        // number (integer), and converts it to the other
        // """

        // parse the string and ensure it follows the right format
        let rn_upper = roman_numeral.to_uppercase();
    
        // # split numeral into a list of letters
        let mut numeral_split: Vec<&str> = rn_upper.split("").collect();
    
        // remove empty indexes at beginning and end
        numeral_split.retain(|s| !s.is_empty());
    
        // # convert each roman numeral into its integer/arabic form
        let mut integer_split: Vec<i64> = vec![];
        for s in numeral_split {
            let v = match s {
                "I" => 1,
                "V" => 5,
                "X" => 10,
                "L" => 50,
                "C" => 100,
                "D" => 500,
                "M" => 1000,
                _ => unimplemented!()
            };
    
            integer_split.push(v);
        }
    
        // # # items used for looping around list of integers
        let output_length = integer_split.len();
        let mut accumulator: Vec<i64> = vec![];
        let mut i = 0;
    
        
        while i < output_length {
    
            // # get the current number
            let number = integer_split[i];
    
            // # if only one roman numeral is present
            if output_length == 1 {
                accumulator.push(number)
    
            // # Rule 2: if we are on the last digit, then we cannot look ahead
            } else if i + 1 == output_length {
                accumulator.push(number);
            } else {
                // # Rule 1: get the next number in the list
                let next_number = integer_split[i + 1];
                
                // # Rule 3: when next number is bigger, subtract current from it
                if number < next_number {
                    let sub_value = next_number - number;
                    accumulator.push(sub_value);
    
                    // # Rule 4: due to subtraction, we need to skip a digit
                    i += 1
                    
                } else {
                    // # Rule 5: when next is not bigger, add it normally
                    accumulator.push(number);
    
                    
                }
            }
    
            i += 1
        }
    
        let sum: i64 = accumulator.iter().sum();

        let output = format!("{sum}");

        self.output = Ok(output);

        self
    }

    pub fn convert_integer(&mut self, i: &i64) -> &mut Self {

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

        self.output = Ok(final_string);

        self
    }
}

fn create_roman_numeral_string(roman_numeral: &str, amount: i64) -> String {
    
    vec![roman_numeral; amount as usize].join("")

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