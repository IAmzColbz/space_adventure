pub mod rlib;

fn main() {
    // Introduction

    println!("I am an 'indentured servant' aboard the Galactic Zorda.
    I have always considered myself a slave, but the ship owners call me 'servant' to avoid
    being prosecuted under intergalactic law, which prohibits slavery. The Zorda was built in 
    year 3388 when Jefferson Davis commissioned the ship to transport his, 'assets.'
    Jefferson was also a well known and feared politician of the galactic empire, who dabbled in slave trade
    and commandeering 'unlicenced' peasant trade ships.");
    println!("The ship is now owned by his son, Robert E. Davis III, almost 50 years later. I was imprisoned
    8 years ago today and spent 6 of those years on a empire sanctioned prison planet named ConV71.
    However 6 years into my 20 year prison sentence, I was misplaced by the warden. I found myself tied and
    gagged in the back of a windowless transport van with 5 others. I only knew this after they opened the 
    van doors to show off the slavers newest subjects. They pulled us out of the back of the van by the 
    backs of our tattered prison rags, letting us hit the ground with little regard or care.");
    println!("After exchanging a few plasteele cards that worked as a form of underground currency, they dragged 
    us into their hulking transport ship. It's thrusters alone  where larger than my old home. This was the
    start of the next 2 years of my life where beatings were common, food was scarce, death was common, and hope
    was lost. I spent my fair share of time sitting on the steel flooring of the cell I was thrown into wonder what
    life could have been. If I didn't speak out against Jefferson and his underhanded practices. My article was 
    evaporated from the newstream before it the eyes of a single person, and thus my arrest was swift yet quiet.");
    println!("That Brings me to today, sitting on cold steel grating, half starving, half beaten. I am going to die
    in here. I'll starve. I'll disappear when the slavers don't think I'm productive enough. I'll be dropped into
    the void of space with no more than my brittle bones and searing hatred.");

    let mut output: String;

    // Game Loop init
    loop {
        let command = rlib::get_input();
        output = rlib::update_state(&command);
        rlib::update_screen(output);

        if matches!(command, rlib::Command::Quit) {
            break;
        }
    }
}
