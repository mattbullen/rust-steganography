## Steganography using the Rust language

A hobby project to learn the [Rust](https://doc.rust-lang.org/book/second-edition/) language. This project encodes a message inside the color/alpha channels of a PNG image's pixels, then later decodes the message to a terminal window.

To encode an image, build and run with:

```[height] [width] [decode password] [.txt file with the message] [file name for the PNG]```

![encode](https://user-images.githubusercontent.com/7276226/27777062-738bb738-5f64-11e7-8ccd-ae297faa4dfa.png)

Be sure to save the final password from the terminal. This is needed to decode the message.

Encoding has two steps. The first step encodes the message characters in their numeric (u8) form inside the color/alpha channels of an intermediate PNG that has been filled with random noise. The noise is there to make it harder to determine which pixels and which channels hold the message information - the only way to know which ones count is to have the password, which sets up the decoding pattern. The intermediate PNG:

![raw_encoded_mask](https://user-images.githubusercontent.com/7276226/27777068-7cd3deba-5f64-11e7-98a9-e7033940eeca.png)

The second step converts the numeric values of every pixel in the intermediate image into binary, splits the binary values into individual digits, then applies them to a "mask" file, or an ordinary-looking image. If a binary digit is 0, then one of the channels of the corresponding mask pixel is adjusted (if needed) to hold an even value. If 1, then odd. The visual changes to the image are very slight and usually imperceptible in images with a large variety of bold colors. For example:

![encoded_mask](https://user-images.githubusercontent.com/7276226/27777071-7ffedc16-5f64-11e7-95fe-cbf8a1f252b1.png)

To decode, much the same:

```[decode password] [PNG's file name]```

![decode](https://user-images.githubusercontent.com/7276226/27777065-76441664-5f64-11e7-9604-3b8ecbe0ceb9.png)
