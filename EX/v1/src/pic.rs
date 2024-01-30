/*
This is for showing mistres ass
*/
use curl::easy::Easy;
pub fn ass_show() {
    let url = "https://snips.sh/f/CRm5avdsZP?r=1";
    let mut easy = Easy::new();
    easy.url(url);
    easy.get(true);

    let mut buffer = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                buffer.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    println!("{}", String::from_utf8(buffer).unwrap());
}
