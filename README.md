# redis-key
Extracts value for given key from redis, written in Rust 🦀

This tool helps us to extract value for given key, as redis windows, doesn't support extracting value for given key

## Commands to run

```cmd
key=redis_key ./redis-key.exe
```

## by default it uses redis url as following
```cmd
redis://localhost:6379
```

## using custom url
```cmd
key=redis_key URL=redis://localhost:6379 ./redis-key.exe
```


## Download executables from below
[download](https://github.com/saiumesh535/redis-key/releases)