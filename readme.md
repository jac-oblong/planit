# planit

**planit** is a project management application. It is built for my own use cases, but others
may find it useful as well. It really only exists because I wanted an offline project 
management / planning application (that was preferably terminal-based) and I did not find
any existing solutions that worked for my needs.

[!NOTE]
This project is not feature complete and is currently on hold. It is unknown when it will be resumed.

## Terminology

Given the name of the application, it only makes sense for the names of things to be celestial
in nature.

| **Term**   | **Meaning**                                                                   |
|:-----------|:------------------------------------------------------------------------------|
| **Comet**  | Comets are meant for bugs / interrupting tasks.                               |
| **Planet** | Planets are meant for normal tasks.                                           |
| **Star**   | Stars contain a collection of other celestial bodies (including other Stars). |

## Environmental Variables

| **Variable**       | **Use**                                                    |
|:-------------------|:-----------------------------------------------------------|
| `PLANET_DATA`      | The full path to the directory to be used for storing data |
| `PLANET_CACHE`     | The full path to the directory to be used for caching      |
| `PLANET_LOG_LEVEL` | The log level to use                                       |

## Command Line Interface

**planit** does support a command line interface, but some of the more complex features are not available through it.

``` shell
planit <subcommand>
```

| **SubCommand Name**    | **SubCommand Action**                                           |
|:-----------------------|:----------------------------------------------------------------|
| `init`                 | Initializes a new **planit** `Galaxy` in the current directory. |
| `list`                 | Lists all celestial bodies in the `Galaxy`.                     |
| `new <celestial body>` | Creates a new object of type `<celestial body>`.                |
