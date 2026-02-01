# Program Output

```text


Messages: 1000000000


sync channel :

crossbeam::unbounded: 36378 ms
kanal::unbounded: 29242 ms
crossfire::unbounded: 30699 ms
std::channel (unbounded): 31404 ms
ringbuffer_spsc: 1570 ms
thingbuf::bounded_blocking: 19907 ms
ringbuf bounded: 9435 ms


async channel :

tachyonix::unbounded: 14552 ms
kanal::unbounded_async: 11503 ms
flume::unbounded: 29117 ms
crossfire::unbounded_async: 30399 ms
tokio::unbounded: 41890 ms
thingbuf::bounded_async: 39760 ms


```
