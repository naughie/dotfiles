def Pry.set_color sym, color
  CodeRay::Encoders::Terminal::TOKEN_COLORS[sym] = color.to_s
  { sym => color.to_s }
end

Pry.config.color = true

Pry.config.prompt = proc do |obj, nest_level, _pry_|
  kaomoji = '(๑o_o)'
  "\001\e[38;5;186m\002#{kaomoji}@#{Pry.config.prompt_name}(#{Pry.view_clip(obj)})\001\e[0m\002< "
end
