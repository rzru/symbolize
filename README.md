# symbolize!

This crate allows you to convert raster images into their symbolic versions! Also available as binary.

## Preview

![Preview](/docs/preview.gif "Binary usage preview")

## Usage as a crate

All needed information is available on [docs.rs](https://docs.rs/symbolize)

## Usage as binary

```
> cargo install symbolize

> symbolize --help
symbolize! 0.1.2
rzru <rzzzzru@gmail.com>
Converts raster images to their symbolic view

USAGE:
    symbolize [OPTIONS] <PATH>

ARGS:
    <PATH>    Path to the original picture

OPTIONS:
    -c, --colorize             Flag that shows should output be colorized for a terminal or not. Not
                               recommended to use it with anything but terminals with rgb support
    -f, --filter <FILTER>      Filter type. One of: nearest, triangle, catmull_rom, gaussian,
                               lanczos3. More about differences:
                               https://docs.rs/image/latest/image/imageops/enum.FilterType.html
                               [default: triangle]
    -h, --help                 Print help information
    -s, --symbols <SYMBOLS>    Defines symbols that will be used to fill the picture (in priority
                               order) [default: *@#&]
        --scale <SCALE>        Defines scale of symbolized picture relatively to the original
                               [default: 1]
    -V, --version              Print version information
    
> symbolize ferris.png --scale=0.05 -s=" @#&" --filter=nearest

                                                                                                                  
                                                            @@                                                          
                                                  @@@@    @@@@      @@                                                  
                                          @@@@    @@@@@@  @@@@@@  @@@@@@    @@                                          
                                          @@@@@@  @@@@@@@@@@@@@@@@@@@@@@  @@@@@@                                        
                                    @@    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    @@                                  
                                  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@                                  
                                  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@                      @@          
          @@                @@@@  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  @@@@              @@@@      @@  
    @@    @@@@              @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@            @@@@@@      @@  
  @@@@    @@@@@@            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@          @@@@@@@@    @@@@  
  @@@@    @@@@@@@@          @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@          @@@@@@@@    @@@@  
  @@@@@@  @@@@@@@@    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    @@@@@@@@  @@@@@@  
  @@@@@@@@@@@@@@@@      @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    @@@@@@@@@@@@@@    
  @@@@@@@@@@@@@@@@      @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@      @@@@@@@@@@@@@@    
    @@@@@@@@@@@@      @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    @@@@@@@@@@      
      @@@@@@@@@@  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  @@@@@@@@        
          @@@@@@    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@&&&&    @@@@@@@@@@&&&&    @@@@@@@@@@@@@@@@@@@@@@@@@@  @@@@            
            @@@@@@    @@@@@@@@@@@@@@@@@@@@@@@@@@@@&&&&      @@@@@@  &&&&    @@@@@@@@@@@@@@@@@@@@@@@@  @@@@              
              @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@##&&      @@@@@@  &&      @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@              
                @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@          @@@@@@          @@@@@@@@@@@@@@@@@@@@@@@@@@@@                
                @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@        @@@@@@@@@@        @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@              
              @@@@@@@@####@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@######@@@@@@@@            
              @@@@@@@@  ######@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  @@@@@@@@@@@@@@@@@@@@@@####  ####  @@@@@@            
                @@@@@@  ####    ######@@@@@@@@@@@@@@@@@@@@@@      @@@@@@@@@@@@@@@@@@######      ##    @@@@              
                  @@@@@@  ##          ##########@@@@@@@@@@@@@@@@@@@@@@@@@@@@########            ##  @@@@@@              
                    @@@@    ##                    ########################                    ##    @@@@                
                      @@@@    ##                                                                  @@@@                  
                        @@                                                                  ##    @@                    
                          @@                                                                      @@                    
                            @@                                                                  @@                      
                            @@                                                                                          
```

## License
[MIT](https://opensource.org/licenses/MIT)
