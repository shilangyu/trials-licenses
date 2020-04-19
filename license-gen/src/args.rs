use std::path::PathBuf;
use structopt::StructOpt;

/// A license generator for Trials Licenses
#[derive(StructOpt, Debug)]
#[structopt(name = "license-gen")]
pub struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Player's nickname
    #[structopt(short, long)]
    pub nickname: String,

    /// Output image path
    #[structopt(short, long, parse(from_os_str))]
    pub output: PathBuf,
}

/*
set nickname PL-k4kshi-TFG

# create nickname image
convert -background none -fill black -font /home/shilangyu/.local/share/fonts/JetBrainsMono-Regular.ttf -pointsize 72 label:$nickname /tmp/$nickname.png


set spacing 300

# overlay components
convert license.png \
    # bikes
    armadillo.png -geometry +0+0 -composite \
    tango.png     -geometry +0+$spacing -composite \
    bronco.png     -geometry +0+(math "$spacing * 2") -composite \
    jackal.png     -geometry +$spacing+0 -composite \
    mantis.png     -geometry +$spacing+$spacing -composite \
    marauder.png     -geometry +$spacing+(math "$spacing * 2") -composite \
    riptide.png     -geometry +(math "$spacing * 2")+0 -composite \
    berzerker.png     -geometry +(math "$spacing * 2")+$spacing -composite \
    phantom.png     -geometry +(math "$spacing * 2")+(math "$spacing * 2") -composite \
    out.png


*/
