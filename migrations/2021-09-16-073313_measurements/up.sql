CREATE TABLE measurements
(
 measurement_Time time with time zone NOT NULL,
 temp1            decimal NOT NULL,
 temp2            decimal NOT NULL,
 temp3            decimal NOT NULL,
 temp4            decimal NOT NULL,
 bme_temp1        decimal NOT NULL,
 bme_temp2        decimal NOT NULL,
 pressure1        decimal NOT NULL,
 pressure2        decimal NOT NULL,
 RH1              decimal NOT NULL,
 RH2              decimal NOT NULL,
 Altitude1        decimal NOT NULL,
 Altitude2        decimal NOT NULL,
 Image_name       character varying(50) NULL,
 CONSTRAINT PK_5 PRIMARY KEY ( Measurement_Time )
);
