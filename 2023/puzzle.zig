//usr/bin/env zig run $0 -- "$@"; exit
const std = @import("std");
const io = @import("io");

pub fn readFile(path: [100]u8) void {
    var file = try std.fs.cwd().openFile(path, .{});
    defer file.close();
    var buf_reader = io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();
    var buf: [1024]u8 = undefined;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        std.debug.print(line, .{});
    }
}

pub fn main() void {
    readFile("example.txt");
    readFile("input.txt");
}
