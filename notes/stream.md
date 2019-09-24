

first attempt to read from a stream...
```
    let mut response = String::new();
    stream.read_line(response).await?;
    println!("{}", response);
```

failed with this error:

```
error[E0599]: no method named `read_line` found for type `tokio_net::tcp::stream::TcpStream` in the current scope
  --> src/main.rs:32:12
   |
32 |     stream.read_line(response).await?;
   |            ^^^^^^^^^
   |
   = note: the method `read_line` exists but the following trait bounds were not satisfied:
           `tokio_net::tcp::stream::TcpStream : tokio_io::io::async_buf_read_ext::AsyncBufReadExt`

```