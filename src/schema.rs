table! {
    measurements (pi_id, measurement_time) {
        pi_id -> Bpchar,
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
        image_data -> Array<Float4>,
    }
}
