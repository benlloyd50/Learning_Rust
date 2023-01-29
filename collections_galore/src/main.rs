fn main() {
    /* Challenge 1:
       Given a list of integers, use a vector and return the 
       median (when sorted, the value in the middle position) 
       and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    */
    // let mut input = vec![3,7,9,2,3,7,5,8,3,10,6,2,9,2,6,3,7,1,4,8,7,7,7,7,7,7,7,7];

    // input.sort();
    // let middle = input.len() / 2;

    // println!("{:?}", input);
    // println!("{}", input[middle]);

    /*  Challenge 2:
        Convert strings to pig latin. 
        The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
        Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
        Keep in mind the details about UTF-8 encoding!
     */
    use unicode_segmentation::UnicodeSegmentation;
    let input = String::from("A Apple");

    let mut output = Vec::new();
    for word in input.split_whitespace() {
        let letters = &word.graphemes(true).collect::<Vec<&str>>();
        
        let first_letter = &letters[0].to_lowercase();
        if vec!["a", "e", "i", "o", "u"].contains(&first_letter.as_str()) {
            if letters.len() == 1 {
                output.push(format!("hay"));
            } else {
                output.push(format!("{word}-hay"));
            }
        } else { //constanants
            if letters.len() == 1 {
                output.push(format!("{first_letter}ay"));
            }
            else {
                let end_of_word = &letters[1..];
                output.push(format!("{end_of_word:?}-{first_letter}ay"));
            }
        }
    }

    println!("{:?}", output);




}
