## Image Sorter

This project is for the generic sorting of images. 
As a data scientist, and a developer,  we have all run into the problem of data not being clean enough, sorting techniques are arguabely barbric, and lack of data for certain types of data is precious. 
This has lead to a possible solution to this problem, using Image Hashing. 
This has been purposed before, but at an argueable cost, image hashing is argueably processing insensitive.
Rust, the language of fearless concurrency seems an awesome fit for this task as its memory sensitive making it great for embedded devices, and one of the fastest languages as of today making it an awesome parter for big data.
The end goal of this application to is give you such percise accuracy, that you can run this on a dataset intended for a deep learning application pre-training. While its a first pass, this application will start with folder structure only sorting but plan to include other sorting methods in the future. 


## Getting Started

As noted in the description this is a rust based application, so downloading the lastest stable version of rust, and run the standard commands and you are ready to go. 
If you are used to running rust applications, you know what to do. 
If not, consider our application downloads on our github page, or keep reading to make the application yourself. 


### Prerequisites

What things you need to install the software and how to install them

rust and its package manager cargo rust 1.30+

```
cargo --release build
```

or if you just want a quick build for testing changes

```
cargo  build
```

NOTE that not specifying a build artifact will produce a debug build. This produces a non-optimized version of the code, that in turn will be signifigantly more memory intensive, cpu intensive, and slower than its release counterpart. 

### Installing

This app can be system installed manually by placing it appropriate system32 folders, and /usr/bin folders. However the app can just be called manually with a path just as effectivly. 

## Running the tests

By default, cargo will run test during builds, if you just want to run tests, they can be run by the following command

```
cargo test
```

which will run all tests, or a specific test  by running with the name of the test file as an arg. 

```
cargo test testname
```

### Break down into end to end tests

Explain what these tests test and why

```
Give an example
```

### And coding style tests

Rust has its own very speciifc coding style, which the compiler will complain about. If there are style complaints, it will not be merged to master. No Exceptions. 


## Deployment

The executable, should be suffficient for all use cases including AWS and various cloud deployments. 
Future support for sorting files within s3 buckets is coming, however it should be noted that this will require a similar but seprate logical set for handling minimialistic reads and writes else cost may become an issue. However it should be noted that any dataset will still need to be a min of O(n) so accessing 200+gb of storage stored and accessed would still be less than $5 at time of writing. 

## Built With

* [rust](http://www.dropwizard.io/1.0.2/docs/) - The web framework used
* [cargo](https://maven.apache.org/) - Dependency Management
* [ImgHash](https://rometools.github.io/rome/) - Used to generate RSS Feeds

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/your/project/tags). 

## Authors

* **Alexander Montgomery** - *Initial work* - [PurpleBooth](https://github.com/PurpleBooth)

See also the list of [contributors](https://github.com/your/project/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* Hat tip to Austin Bonander who implemented the ImageHash library in the open source
