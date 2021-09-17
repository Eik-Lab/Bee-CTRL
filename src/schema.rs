table! {
    measurements (pi_id, measurement_time) {
        pi_id -> Int4,
        measurement_time -> Timestamptz,
        temp1 -> Float4,
        temp2 -> Float4,
        temp3 -> Float4,
        temp4 -> Float4,
        bme_temp1 -> Float4,
        bme_temp2 -> Float4,
        pressure1 -> Float4,
        pressure2 -> Float4,
        rh1 -> Float4,
        rh2 -> Float4,
        altitude1 -> Float4,
        altitude2 -> Float4,
        image_name -> Array<Float4>,
    }
}
