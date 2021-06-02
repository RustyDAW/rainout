use rusty_daw_io::{
    AudioDeviceConfig, AudioServerConfig, DevicesInfo, MidiDeviceConfig, MidiServerConfig,
    ProcessInfo, RtProcessHandler, StreamInfo,
};

fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();

    let info = DevicesInfo::new();

    dbg!(info.audio_servers_info());
    dbg!(info.midi_servers_info());

    let audio_config = AudioServerConfig {
        server: String::from("Jack"),
        system_duplex_device: String::from("Jack"),
        system_half_duplex_in_device: None,
        system_half_duplex_out_device: None,

        create_in_devices: vec![AudioDeviceConfig {
            id: String::from("audio_in"),
            system_ports: vec![
                String::from("system:capture_1"),
                String::from("system:capture_2"),
            ],
        }],
        create_out_devices: vec![AudioDeviceConfig {
            id: String::from("audio_out"),
            system_ports: vec![
                String::from("system:playback_1"),
                String::from("system:playback_2"),
            ],
        }],

        sample_rate: None,
        buffer_size: None,
    };

    let midi_config = MidiServerConfig {
        server: String::from("Jack"),
        create_in_devices: vec![MidiDeviceConfig {
            id: String::from("midi_in"),
            system_port: String::from("system:midi_capture_2"),
        }],
        create_out_devices: vec![MidiDeviceConfig {
            id: String::from("midi_out"),
            system_port: String::from("system:midi_playback_1"),
        }],
    };

    let stream_handle = rusty_daw_io::spawn_rt_thread(
        &audio_config,
        Some(&midi_config),
        Some(String::from("testing")),
        MyRtProcessHandler {},
        |e| {
            println!("Fatal stream error: {:?}", e);
        },
    )
    .unwrap();

    dbg!(stream_handle.stream_info());

    // Wait for user input to quit
    println!("Press enter/return to quit...");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).ok();
}

struct MyRtProcessHandler {}

impl RtProcessHandler for MyRtProcessHandler {
    fn init(&mut self, stream_info: &StreamInfo) {}
    fn process(&mut self, proc_info: ProcessInfo) {}
}
