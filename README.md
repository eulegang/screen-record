
# Screen Record

A binary for recording the screen and make videos or potentially stream to services

works via ffmpeg and other linux utilities


## Prototype

```
ffmpeg -video_size 2560x1440 -framerate 25 -f x11grab -i :0.0+2256+0 output.mp4
```


