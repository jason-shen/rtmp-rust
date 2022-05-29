use gstreamer as gst;
use gst::prelude::*;

fn main() {
    gst::init().unwrap();
    // gst-launch-1.0 videotestsrc ! x264enc ! flvmux streamable=true ! rtmpsink location=rtmp://localhost:1935/live/jason\?key=password

    // rtmp uri
    let rtmp_uri = "rtmp://localhost:1935/live/jason?key=password";

    // create gstreamer elements
    let source = gst::ElementFactory::make("videotestsrc", Some("source"))
        .expect("Could not create sink element videotestsrc");
    let x264enc = gst::ElementFactory::make("x264enc", Some("x264enc"))
        .expect("Could not create sink element x264enc");
    let flvmux = gst::ElementFactory::make("flvmux", Some("flvmux"))
        .expect("Could not create sink element flvmux");
    let video_sink = gst::ElementFactory::make("video_sink", Some("video_sink"))
    .expect("Could not create sink element video_sink");

    // set properties
    flvmux.set_property("localtion", true);
    video_sink.set_property_from_str("localtion", rtmp_uri);

    // Create empty pipeline
    let pipeline = gst::Pipeline::new(Some("live-pipeline"));

    // Build the pipeline
    pipeline.add_many(&[&source, &x264enc, &flvmux, &video_sink]).unwrap();
    gst::Element::link_many(&[&source, &x264enc, &flvmux, &video_sink])
        .expect("Elements could not be linked!");

    // start playing
    pipeline.set_state(gst::State::Playing)
        .expect("Unable to set the pipeline playing state");
        
    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;
        match msg.view() {
            MessageView::Error(err) => {
                println!("Error recieved from element {:?} {}", 
                err.src().map(|s| s.path_string()),
                err.error()
            );
                break;
            }
            MessageView::StateChanged(state_changed) => {
                if state_changed
                .src()
                .map(|s| s == pipeline)
                .unwrap_or(false) {
                    println!(
                        "Pipeline state changed from {:?} to {:?}",
                        state_changed.old(),
                        state_changed.current()
                    )
                }
            }
            MessageView::Eos(_) => break,
            _ => (),
        }
    }    

}
