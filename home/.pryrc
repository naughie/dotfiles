def Pry.set_color sym, color
  CodeRay::Encoders::Terminal::TOKEN_COLORS[sym] = color.to_s
  { sym => color.to_s }
end

Pry.config.color = true

Pry.config.prompt = Pry::Prompt.new(
  "custom",
  "my custom prompt",
  [proc { |obj, nest_level, _pry_|
    kaomoji = '(๑o_o)'
    "\001\e[38;5;186m\002#{kaomoji}@#{Pry.config.prompt_name}(#{Pry.view_clip(obj)})\001\e[0m\002< "
  }]
)

ENV['PAGER'] = 'less'
