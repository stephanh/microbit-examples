# Temperature measurement using a thermistor

Measures the temperature using a thermistor MF52A5k3950 (I thin) thermistor in
series with a 5k4 resistor. The voltage is measured using the Microbit's GPIO
PIN0 (AIN4, nRF51 pin 3).

This uses an implementation of the [ADC
API](https://docs.rs/embedded-hal/0.2.2/embedded_hal/adc/trait.OneShot.html)
from embeddded-hal for the nRF51.
