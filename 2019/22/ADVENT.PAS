
program advent;

uses Strings;

const size = 10007;
type deck = array[0..size-1] of integer;
var cards: deck;
var buffer: deck;

procedure init;
var i : integer;
begin
  for i := 0 to size-1 do
    cards[i] := i;
end;

procedure reverse;
var i, temp : integer;
begin
  for i:=0 to ((size-1) div 2) do begin
    temp := cards[i];
    cards[i] := cards[(size-1)-i];
    cards[(size-1)-i] := temp;
  end;
end;

procedure cut(n: integer);
var i : integer;
begin
  if n < 0 then
    n := size + n;
  for i := 0 to (n-1) do
    buffer[(size-n)+i] := cards[i];
  for i := n to (size-1) do
    buffer[i-n] := cards[i];
  for i := 0 to size-1 do
    cards[i] := buffer[i];
end;

procedure increment(n: integer);
var src, dst : integer;
begin
  dst := 0;
  for src := 0 to (size-1) do begin
    buffer[dst] := cards[src];
    dst := (dst + n) mod size;
  end;
  for src := 0 to size-1 do
    cards[src] := buffer[src];
end;

procedure writedeck;
var i: integer;
begin
  for i := 0 to size-1 do
  begin
    write(cards[i]);
    write(' ');
  end;
  writeln;
end;

procedure process;
var f: text;
var line: string;
var s: pchar;
var cmp, param, code: integer;
var more: boolean;
begin
  assign(f, 'C:\INPUT.TXT');
  reset(f);
  more := true;
  while more do begin
    readln(f, line);
    writeln(line);
    if length(line) = 0 then
      more := false
    else begin
      getmem(s, length(line)+1);
      strpcopy(s, line);
      cmp := strcomp(s, 'deal into new stack');
      if cmp = 0 then begin
        reverse
      end else if cmp < 0 then begin
        val(s+4, param, code);
        cut(param);
      end else begin
        val(s+20, param, code);
        increment(param);
      end;
      freemem(s, length(line)+1);
    end;
  end;
  close(f);
end;

procedure search;
var i: integer;
begin
  for i := 0 to size-1 do
    if cards[i] = 2019 then
      writeln(i);
end;

begin
  init; process; search; readln;
end.
