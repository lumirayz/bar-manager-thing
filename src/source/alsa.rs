extern crate alsa_sys as alsa;

use std::ffi::CString;
use std::ptr;

pub fn get_volume() -> f64 {
    let mut min = 0;
    let mut max = 0;
    let mut vol = 0;
    unsafe {
        // TODO: use proper bindings
        let card_name = CString::new("default").unwrap();
        let selem_name = CString::new("Master").unwrap();
        let mut hnd = ptr::null_mut();
        if alsa::snd_mixer_open(&mut hnd, 0) != 0 {
            panic!("Cannot open ALSA mixer.");
        }
        if alsa::snd_mixer_attach(hnd, card_name.as_ptr()) != 0 {
            panic!("Cannot attach to ALSA device.");
        }
        alsa::snd_mixer_selem_register(hnd, ptr::null_mut(), ptr::null_mut());
        alsa::snd_mixer_load(hnd);
        let mut sid = ptr::null_mut();
        alsa::snd_mixer_selem_id_malloc(&mut sid);
        alsa::snd_mixer_selem_id_set_index(sid, 0);
        alsa::snd_mixer_selem_id_set_name(sid, selem_name.as_ptr());
        let mut elem = alsa::snd_mixer_find_selem(hnd, sid);
        alsa::snd_mixer_selem_get_playback_volume_range(elem,
            &mut min,
            &mut max);
        alsa::snd_mixer_selem_get_playback_volume(elem, alsa::SND_MIXER_SCHN_FRONT_LEFT,
            &mut vol);
        alsa::snd_mixer_close(hnd);
    }
    let pct : f64 = vol as f64 / (max - min) as f64 * 100.0;
    pct
}
