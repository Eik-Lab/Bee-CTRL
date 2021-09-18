CREATE TABLE measurements
(
 pi_id int NOT NULL,
 measurement_time timestamptz NOT NULL,
 temp1            float4 NOT NULL,
 temp2            float4 NOT NULL,
 temp3            float4 NOT NULL,
 temp4            float4 NOT NULL,
 bme_temp1        float4 NOT NULL,
 bme_temp2        float4 NOT NULL,
 pressure1        float4 NOT NULL,
 pressure2        float4 NOT NULL,
 RH1              float4 NOT NULL,
 RH2              float4 NOT NULL,
 Image_data       float4[] NOT NULL,
PRIMARY KEY (pi_id, measurement_time)
);
