use leptos::*;

use rusty_romans::output::{Output, parse_value_to_integer};


#[component]
fn App(cx: Scope) -> impl IntoView {
    let (read_value, set_value) = create_signal(cx, Ok(0.to_string()));

    

    let validated_value = move |event| {
        let ev = event_target_value(&event);

        let mut output = Output::init();

        let validated_event = match parse_value_to_integer(&ev) {
            Ok(int) => output.validate_integer_input(int),
            Err(_) => output.validate_roman_numeral_input(&ev),
        };

        

        let f = validated_event.clone();

        set_value.set(f.output)
    };

    

    let converted_value = move || {

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
