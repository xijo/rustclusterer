# rustclusterer
Cluster algorithm of [clumpy](https://github.com/xijo/clumpy), implemented in rust.

My goal was to find out if I could speed up the gem by implementing the calculation heavy bits in a performant language. The problem is, that passing the data in and out is pretty slow. I used JSON, but the deserialization takes quite a while in rust. Therefor the ruby version is still faster, yet.
