// Copyright 2025 Kevin Ludwig
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

const rb_tree = @import("util/rb_tree.zig");

const Self = @This();

const Used = struct {
    addr: usize,
    size: usize,

    fn compare(a: Used, b: Used) rb_tree.Order {
        return if (a.addr < b.addr) .lt else if (a.addr > b.addr) .gt else .eq;
    }
};

const UsedList = rb_tree.Tree(Used, Used.compare);

used: UsedList = .{},
