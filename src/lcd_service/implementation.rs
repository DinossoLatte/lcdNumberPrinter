use std::string::String;

fn render_two_vertical_lines(size: usize) -> String {
    let mut str = String::from("|");
    str.push_str(&" ".repeat(size as usize));
    str.push_str("|");
    return str;
}

fn render_horizontal_line(size: usize) -> String {
    let mut str = String::from(" ");
    str.push_str(&"-".repeat(size as usize));
    str.push_str(" ");
    return str;
}

fn render_left_vertical_line(size: usize) -> String {
    let mut str = String::from("|");
    str.push_str(&" ".repeat(size+1));
    return str;
}

fn render_right_vertical_line(size: usize) -> String {
    let mut str = String::from(" ").repeat(size+1);
    str.push_str("|");
    return str;
}

fn render_character(line: u8, number: u64, size: usize) -> String {
    let res = match line {
        0 => {
            match number {
                0|2|3|5|6|7|8|9 => {
                    render_horizontal_line(size)
                },
                _ => {
                    return " ".repeat(size+2)
                }
            }
        },
        1 => {
            match number {
                0|4|7|8|9 => {
                    render_two_vertical_lines(size)
                },
                1|2|3 =>{
                    render_right_vertical_line(size)
                },
                _ => {
                    render_left_vertical_line(size)
                }
            }
        },
        2 => {
            match number {
                2|3|4|5|6|8|9 => {
                    render_horizontal_line(size)
                },
                _ => {
                    return " ".repeat((size+2) as usize)
                }
            }
        },
        3 => {
            match number {
                0|6|8 => {
                    render_two_vertical_lines(size)
                },
                1|3|4|5|7|9 => {
                    render_right_vertical_line(size)
                },
                _ => {
                    render_left_vertical_line(size)
                }
            }
        },
        _ => {
            match number {
                0|2|3|5|6|8 => {
                    render_horizontal_line(size)
                },
                _ => {
                    " ".repeat((size+2) as usize)
                }
            }
        }
    };
    return res;
}

#[warn(unused_assignments)]
pub fn render_lcd<'b>(line_size: &'b str, numbers: u64) -> Result<String, String> {
    let mut res = String::new();
    // Primero necesitamos adaptar los argumentos a un sistema más manejable
    // line_size interesa que sea un número
    let size_res = line_size.parse::<u8>();
    if size_res.is_ok() {
        let size = size_res.unwrap();
        // Necesitamos definir también algunas lineas especiales
        let middle = 1+size;
        let last = 2*size + 2;
        // Convertimos el número en un array de números
        let array_number: Vec<u64> = numbers.to_string().chars().map(|x| x.to_string().parse::<u64>().unwrap()).collect();
        // Ahora iniciamos el for que rastreará por linea
        // Iteraremos por cada una de las lineas del LCD, que son 2 horizontales y verticales.
        for line in 0..(last+1) {
            // Para cada uno añadiremos la cantidad de lineas a aplicar por caracter
            // Realizamos un switch donde veremos el tipo de caracter a emitir
            for number in &array_number {
                // El tipo de linea será diferente a la linea a renderizar
                let line_type: u8 = {
                    // Si es la posición 0, será la primera linea
                    if line == 0 {
                        0
                    } else if line < middle && line > 0 {
                        1
                    } else if line == middle {
                        2  
                    } else if line < last && line > middle {
                        3
                    } else {
                        4
                    }
                };
                res.push_str(&(render_character(line_type, number.clone(), size as usize)));
                // Separamos a los caracteres una vez más
                res.push_str(" ");
            }
            res.push_str("\n");
            // Finalmente, generamos un \n para que salte a la siguiente linea
        }
        return Ok(res);
    } else {
        return Err(String::from("Failed to parse line size"));
    }
}