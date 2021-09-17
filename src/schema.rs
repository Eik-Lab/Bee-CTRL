table! {
    measurements (measurement_time) {
        measurement_time -> Timetz,
        temp1 -> Numeric,
        temp2 -> Numeric,
        temp3 -> Numeric,
        temp4 -> Numeric,
        bme_temp1 -> Numeric,
        bme_temp2 -> Numeric,
        pressure1 -> Numeric,
        pressure2 -> Numeric,
        rh1 -> Numeric,
        rh2 -> Numeric,
        altitude1 -> Numeric,
        altitude2 -> Numeric,
        image_name -> Nullable<Varchar>,
    }
}
