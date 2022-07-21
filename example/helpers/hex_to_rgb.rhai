let num = params[0].sub_string(1).parse_int(16);
let r = ((num & 0xff0000) >> 16).to_string();
let g = ((num & 0x00ff00) >> 8).to_string();
let b = (num & 0x0000ff).to_string();

"rgb(" + r + ", " + g + ", " + b + ")"
