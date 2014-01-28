Title: Problem Set 1 Answers
Author: <Sukriti Thapa>

Problem 1.
The user-agent string generated:

User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux i686; rv:26.0) Gecko/20100101 Firefox/26.0

The user-agent seems to be accessing the server through the network which is the internet. 

The Mozilla/5.0 part shows that the user agent is Mozilla based. I believe that the 5.0 shows that it is Mozilla version 5.0. Since I'm using virtual box, I'm not sure what X11 means but I assume that is where it should tell me what system I am running on (Macintosh for example). The next part shows that the host operating system is Linux i586. I'm not sure what rv: 26.0 means or what Gecko/20100101 means either. The line with firefox means that I used Firefox. 

Problem 2.

Rust is a language that is very focused on safety. According to the rust site, Rust has a "sophisticated memory model that encourages efficient data structures and safe concurrency patterns" so it forbids invalid memory accesses. Therefore, Rust thinks it is unsafe to modify a global variable because it results in an unsafe function since a global variable can be used within any scope. Therefore, if the variable was used in a function
