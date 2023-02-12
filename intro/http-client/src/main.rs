use core::str;
use std::str::from_utf8;

use bsc::wifi::wifi;
use embedded_svc::{
    http::{
        client::{Client, Request, RequestWrite, Response},
        Status,
    },
    io::Read,
};
use esp32_c3_dkc02_bsc as bsc;
use esp_idf_svc::http::client::{EspHttpClient, EspHttpClientConfiguration};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let _wifi = wifi(CONFIG.wifi_ssid, CONFIG.wifi_psk)?;

    // TODO your code here
    get("https://espressif.com")?;

    Ok(())
}

fn get(url: impl AsRef<str>) -> anyhow::Result<()> {
    // 1. Create a new EspHttpClient. (Check documentation)
    let mut client = EspHttpClient::new(&EspHttpClientConfiguration {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),

        ..Default::default()
    })?;
    // 2. Open a GET request to `url`
    let request = client.get(url)?;

    // 3. Requests *may* send data to the server. Turn the request into a writer, specifying 0 bytes as write length
    // (since we don't send anything - but have to do the writer step anyway)
    //
    // https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/protocols/esp_http_client.html
    // If this were a POST request, you'd set a write length > 0 and then writer.do_write(&some_buf);

    let writer = request.into_writer(0)?;

    // 4. Turn the writer into a response and check its status. Successful http status codes are in the 200..=299 range.

    let response = writer.into_response()?;
    let status = response.status();
    println!("Response code: {}", status);

    match status {
        200..=299 => {
            let mut reader = response.reader();
            let mut total_bytes = 0;
            loop {
                let mut buffer = [0u8; 256];
                let bytes_read = reader.do_read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                total_bytes += bytes_read;
                let response_string = from_utf8(&buffer[..bytes_read])?;
                print!("{}", response_string);
            }
            println!("\nBytes read: {}", total_bytes);
        }
        300..=399 => {
            todo!()
        }
        400..=499 => {
            todo!()
        }
        500..=599 => {
            todo!()
        }
        _ => anyhow::bail!("Unexpected response code: {}", status),
    }

    Ok(())
}
