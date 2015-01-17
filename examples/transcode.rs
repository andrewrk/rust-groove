#![allow(unstable)]
extern crate groove;

use std::option::Option;

// transcode one or more files into one output file

fn main() {
    let mut stderr = std::io::stderr();
    let args = std::os::args();
    let exe = args[0].as_slice();

    let mut bit_rate_k = 320;
    let mut format_option = Option::None;
    let mut codec = Option::None;
    let mut mime = Option::None;
    let mut output_file_name_option = Option::None;

    groove::init();
    groove::set_logging(groove::Log::Info);

    let playlist = groove::Playlist::new();

    let mut i = 1;
    while i < args.len() {
        let full_arg = args[i].as_slice();
        if full_arg.starts_with("--") {
            let arg = full_arg.slice_from(2);
            if i + 1 >= args.len() {
                print_usage(&mut stderr, exe);
                std::os::set_exit_status(1);
                return;
            } else if arg == "bitrate" {
                i += 1;
                bit_rate_k = args[i].parse().unwrap();
            } else if arg == "format" {
                i += 1;
                format_option = Option::Some(args[i].as_slice());
            } else if arg == "codec" {
                i += 1;
                codec = Option::Some(args[i].as_slice());
            } else if arg == "mime" {
                i += 1;
                mime = Option::Some(args[i].as_slice());
            } else if arg == "output" {
                i += 1;
                output_file_name_option = Option::Some(args[i].as_slice());
            } else {
                print_usage(&mut stderr, exe);
                std::os::set_exit_status(1);
                return;
            }
        } else {
            match groove::file_open(full_arg) {
                Option::Some(file) => {
                    playlist.append(&file, 1.0, 1.0);
                },
                Option::None => {
                    let _ = writeln!(&mut stderr, "Error opening input file {}", full_arg);
                    std::os::set_exit_status(1);
                    return;
                },
            }
        }
        i += 1;
    }
    let output_file_name = match output_file_name_option {
        Option::Some(file_name) => file_name,
        Option::None => {
            print_usage(&mut stderr, exe);
            std::os::set_exit_status(1);
            return;
        },
    };
    let encoder = groove::Encoder::new();
    encoder.set_bit_rate(bit_rate_k * 1000);
    match format_option {
        Option::Some(format) => { encoder.set_format_short_name(format) },
        _ => {},
    }
    groove::finish();
}
/*
int main(int argc, char * argv[]) {
    struct GrooveEncoder *encoder = groove_encoder_create();
    encoder->bit_rate = bit_rate_k * 1000;
    encoder->format_short_name = format;
    encoder->codec_short_name = codec;
    encoder->filename = output_file_name;
    encoder->mime_type = mime;
    if (groove_playlist_count(playlist) == 1) {
        groove_file_audio_format(playlist->head->file, &encoder->target_audio_format);

        // copy metadata
        struct GrooveTag *tag = NULL;
        while((tag = groove_file_metadata_get(playlist->head->file, "", tag, 0))) {
            groove_encoder_metadata_set(encoder, groove_tag_key(tag), groove_tag_value(tag), 0);
        }
    }

    if (groove_encoder_attach(encoder, playlist) < 0) {
        fprintf(stderr, "error attaching encoder\n");
        return 1;
    }

    FILE *f = fopen(output_file_name, "wb");
    if (!f) {
        fprintf(stderr, "Error opening output file %s\n", output_file_name);
        return 1;
    }

    struct GrooveBuffer *buffer;

    while (groove_encoder_buffer_get(encoder, &buffer, 1) == GROOVE_BUFFER_YES) {
        fwrite(buffer->data[0], 1, buffer->size, f);
        groove_buffer_unref(buffer);
    }

    fclose(f);

    groove_encoder_detach(encoder);
    groove_encoder_destroy(encoder);

    struct GroovePlaylistItem *item = playlist->head;
    while (item) {
        struct GrooveFile *file = item->file;
        struct GroovePlaylistItem *next = item->next;
        groove_playlist_remove(playlist, item);
        groove_file_close(file);
        item = next;
    }
    groove_playlist_destroy(playlist);

    return 0;
}
*/


fn print_usage(stderr: &mut std::io::LineBufferedWriter<std::io::stdio::StdWriter>, exe: &str) {
    let _ = write!(stderr, "Usage: {} file1 [file2 ...] --output outputfile [--bitrate 320] [--format name] [--codec name] [--mime mimetype]\n", exe);
}
