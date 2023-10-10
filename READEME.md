# linemerge
A small tool that fixes an issue with certain SSA subtitle where new lines get split accross multiple "Dialogue" events (they should be a single event deliminated by a new line "\N"). 
It also fixes a similar issue where a single event may be split into / span across serveral consecutive events.

For example, improperly formated lines like these: 

```
Dialogue: 0,0:01:58.33,0:02:00.00,Default,,0,0,0,,≪な〜んだ
Dialogue: 0,0:01:58.33,0:02:00.00,Default,,0,0,0,,マッチングアプリの広告か…≫
Dialogue: 0,0:02:00.00,0:02:03.30,Default,,0,0,0,,≪な〜んだ
Dialogue: 0,0:02:00.00,0:02:03.30,Default,,0,0,0,,マッチングアプリの広告か…≫
```

Will get merged into a single line like this: 
```
Dialogue: 0,0:01:58.33,0:02:03.30,Default,,0,0,0,,≪な〜んだ\Nマッチングアプリの広告か…≫
```

# Building
Run: `cargo build --release`

# Usage 
```
linemerge <INPUT FILE> [OUTPUT FILE]

Arguments:
  <INPUT FILE>   Path to input file
  [OUTPUT FILE]  (Optional) Path to output file (default: stdout)
```