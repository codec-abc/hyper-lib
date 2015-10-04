# Summary

This is a sample project to the rust hyper library and use it on the C/C++ side to process only POST request. Goal is to use as a web server for streaming data to a vrpn server.

### warning
The support for https has been deactivated as it is painfull to set up on Windows.

## Instructions

* Build with cargo build --release
* Compile with the matching version on Windows (msvc and not the mingw one)
* Here is an example code that can use it

```
// testRustFFI.cpp : Defines the entry point for the console application.
//

extern "C"
{
	void* start(int);
	char* get_string(void* );
	void free_string(char*);
}

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
	std::cout << "starting" << std::endl;
	void* w = start(3000);
	while (true)
	{

		char* y = get_string(w);
		if (y != NULL)
		{
			std::string str(y);
			std::cout << "getting string from server " << str << std::endl;
			free_string(y);
		}
	}
	std::cin.get();
	return 0;
}
```

* Link against the static lib.
* Be use to use the /MD flag (shared dynamic library for runtime)
* Link against the following system library (see the ouput of cargo build) :
  * shell32.lib
  * ws2_32.lib
  * userenv.lib
