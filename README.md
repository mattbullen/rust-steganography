## Steganography using the Rust language

A hobby project to learn the [Rust](https://doc.rust-lang.org/book/second-edition/) language. The first part encodes a message inside the color channels of a PNG image's pixels; the second part decodes the message from the image.

To encode a message, the terminal command needs these parameters:

```[height] [width] [decode password] [.txt file with the message] [file name for the PNG]```

![encode screenshot](https://user-images.githubusercontent.com/7276226/27767127-0adc2470-5ea1-11e7-819b-842b0629572d.png)

Be sure to save the final password from the terminal. This is needed to decode the message.

The PNG itself:

![encoded](https://user-images.githubusercontent.com/7276226/27767128-2230ff10-5ea1-11e7-9fbb-981989b9d7f1.png)

To decode, much the same:

```[password] [PNG's file name]```

![decode screenshot](https://user-images.githubusercontent.com/7276226/27767131-387e0394-5ea1-11e7-8f6a-230a772fdb33.png)

This is essentially security through obscurity, since you need to know the start coordinates for where the sequence of significant pixels begins, along with the message length, but nothing prevents anyone from trying every possible combination of pixel channels to (eventually) find the message values.
