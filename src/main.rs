use std::{path::PathBuf, fs::File, io::{Write, BufReader, BufRead}};

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Input file
    #[arg(value_name="INPUT FILE")]
    input:  PathBuf,
    /// Output file
    #[arg(value_name="OUTPUT FILE")]
    output: Option<PathBuf>,
}

trait FindNth {
    fn find_nth(&self, n: usize, tgt: char) -> Option<usize>;
}
impl FindNth for &str {
    fn find_nth(&self, n: usize, tgt: char) -> Option<usize> {
        (0..n).try_fold(0, |acc,_| {
            self.get(acc..)
                .and_then(|s| s.find(tgt)
                               .and_then(|x| Some(x + acc + 1)))
        }).and_then(|x| Some(x - 1) )
    }
}

trait WithinNOf {
    fn within_n_of(&self, n: Self, other: Self) -> bool;
}

impl WithinNOf for u64 {
    fn within_n_of(&self, n: Self, other: Self) -> bool {
        *self >= other.saturating_sub(n) 
        && *self <= other.saturating_add(n)
    }
}

fn main() {
    let args = Cli::parse();
    let input_file = File::open(args.input).unwrap();
    let mut output_stream = match args.output {
        Some(path) => Box::new(File::create(path).unwrap()) as Box<dyn Write>,
        None => Box::new(std::io::stdout()) as Box<dyn Write>,
    };
    let lines = BufReader::new(input_file).lines();

    let mut prev_info = String::new();
    let mut line_buffer: Vec<String> = Vec::with_capacity(4);
    let mut combined_lines: Vec<String> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        // Only Process Dialogue events 
        // this progam assumes there will be only Dialogue events after header section
        if  line.len() < 10 || &line.as_bytes()[..10] != b"Dialogue: " {
            writeln!(output_stream, "{line}").unwrap();
            continue;
        }
        let [curr_info, text_content] = split_info_and_text_content(&line);
        if prev_info.len() > 0 &&curr_info.ne(&prev_info)  {
            combined_lines.push(String::from(
                format!("{prev_info}{}", line_buffer.join(r"\N"))
            ));
            line_buffer.clear();
        }
        prev_info.clear();
        prev_info.push_str(&curr_info);
        line_buffer.push(String::from(text_content));
    }
    combined_lines.push(String::from(
        format!("{prev_info}{}", line_buffer.join(r"\N"))
    ));

    let mut combined_lines = combined_lines.iter();
    let first_line = combined_lines.next().unwrap();
    let mut prev_text = get_text_portion(first_line);
    let [mut prev_start_time, mut prev_end_time] = {
        get_times(first_line).map(|t| parse_time_str(t))
    };
    let mut combined_string = String::with_capacity(first_line.len()*2);
    combined_string.push_str(first_line);
    for line in combined_lines {
        let curr_text = get_text_portion(line);
        let [curr_start_time, curr_end_time] = {
            get_times(line).map(|t| parse_time_str(t))
        };
        if ( curr_start_time.within_n_of(500, prev_end_time)
        || (prev_start_time..prev_end_time).contains(&curr_start_time))
        && curr_text.eq(prev_text) {
            // Rework this
            
        } else {
            let pos = combined_string.find(',').unwrap()+1;
            combined_string.replace_range(
                pos..pos+21,
                &(mk_time_str(prev_start_time) + "," + &mk_time_str(prev_end_time))
            );
            writeln!(output_stream, "{combined_string}").unwrap();
            combined_string.clear();
            combined_string.push_str(line);
            prev_start_time = curr_start_time;
        }
        prev_end_time = curr_end_time;
        prev_text = curr_text;
    }
    let pos = combined_string.find(',').unwrap()+1;
    combined_string.replace_range(
        pos..pos+21,
        &(mk_time_str(prev_start_time) + "," + &mk_time_str(prev_end_time))
    );
    writeln!(output_stream, "{combined_string}").unwrap();
}


fn get_text_portion(line: &str) -> &str {
    split_info_and_text_content(line)[1]
}
fn split_info_and_text_content(line: &str) -> [&str;2] {
    let contents_pos = (&line[..]).find_nth(9, ',').unwrap()+1;
    [&line[..contents_pos], &line[contents_pos..]]
}

fn get_times(line: &str) -> [&str;2] {
    let mut iter = line.split(',');
    iter.next();
    let start =  iter.next().unwrap();
    let end = iter.next().unwrap();
    [start, end]
}

fn parse_time_str(time: &str) -> u64 {
    let mut iter = time.split(|c| c == ':' || c == '.');
    let mut millis = 0;
    millis += iter.next().unwrap().parse::<u64>().unwrap() * 3_600_000; // Hours
    millis += iter.next().unwrap().parse::<u64>().unwrap() * 60_000; // Mins
    millis += iter.next().unwrap().parse::<u64>().unwrap() * 1_000;  // Secs
    millis += iter.next().unwrap().parse::<u64>().unwrap() * 10; // Hunds
    millis
}

fn mk_time_str(mut millis: u64) -> String {
    let mut time_string = String::with_capacity(10);
    time_string.push_str(&format!("{:01}:", millis / 3_600_000));
    millis %= 3_600_000;
    time_string.push_str(&format!("{:02}:", millis / 60_000));
    millis %= 60_000;
    time_string.push_str(&format!("{:02}.", millis / 1_000));
    millis %= 1_000;
    time_string.push_str(&format!("{:02}" , millis / 10) );
    time_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_nth() {
        //       00000000001111111111222222222233333333334444444444
        //       01234567890123456789012345678901234567890123456789
        let x = ",hello,there, how, five, six, seven,hi".find_nth(7, ',').unwrap();
        println!("HEEEEEEEEEEEEEEEEEEEEEEEE {x}");
        assert_eq!(
            x,
            35
        );
    }

    #[test]
    fn test_get_times(){
        println!("{:?}",get_times("time1,time2,time3"));
        
    }

    #[test]
    fn test_time_string() {
        let old = "5:12:55.94";
        let millis = parse_time_str("5:12:55.94");
        let new = &mk_time_str(millis)[..];
        println!("{old}\n{millis}\n{new}");
        assert!(old.eq(new));
    }


}