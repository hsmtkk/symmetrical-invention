use anyhow::Result;
use std::io::Read;

fn main(){
    let header = "eyJ6aXAiOiJERUYiLCJhbGciOiJFUzI1NiIsImtpZCI6Ik9TZzhwS0tuSURpaUoweFZZQmo2OHBuSU1hRDloZWw2TzdDMndWNUdteGMifQ";
    let payload = "3ZJLbhNBEIbvUmzn7fg1OwMRJBArggQWKLLaPT2ejnqmR_0YYSIvOAIHQKw4AGw5Dz4HVWMbJVKSFStm1131f1P_X30D0lrIoXKutXkcd8ZGhVxJx1S00tF1G2PdCwMBNMsS8nQ0GGaDJB1k0WQSQMchvwG3bgXkH_5CbM2MqwRTroo4M4V9sjuEdEDSw32yrn0jPzEndfNoI9edLNIpXAXAjShE4yRTb_3yWnBHI5WVNO-EscTJ4ShKohR5dPvUN4US1GOE1d5wcdGPD_tCsLcDXCuFtN0k-AOzRo9I9kpdGoUNB32eYMPhcA_4HO2gniJktegh3lLBW88UjcVqqZAO2x_ftl-_4M1KdqKhSH9__r79-QuuNsFBpMtScnlX9-p4fnpyW3Z8dvlmhiqMZykxuufMkTadTpMwzcJkDBsi3uMlfdzLyd0FWcect31YdauEE7TejnEuG_FMFz2B60I2q962XVsn6v1zw71Wahxps4ppL7GVRcy7jwjgvRIyGhOdQ7sPsB-nFEY0NNvt_LFJc-5NXyKzF7LeIbI0TCboGbGtMKU2NT5mmoVxpw0hC2lbxSjGs5ev3y9e6G6hy8Xp7Hw2x5QwQaXd3NdL0sEgSY6ybPJgfNl_GN84TIf_Lr7BaDiiwga_Pw";
    //let sign = "HOw2bwCjhiqBqFUn-8hf62NxFGtdu3NlAiZt8jJUfFO89C0olE3E2xck9c3woq3HMJ3PFNuNhiuumc9QkdJJZQ";

    let header_decoded = base64::decode(header).unwrap();
    println!("header {}", std::str::from_utf8(&header_decoded).unwrap());

    let payload = payload.replace("_", "/").replace("-", "+");
    let payload_decoded = base64::decode(payload).unwrap();
    let mut decoder = flate2::read::DeflateDecoder::new(&*payload_decoded);
    let mut extended = String::new();
    decoder.read_to_string(&mut extended).unwrap();
    println!("payload {}", extended);
}
