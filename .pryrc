Pry.config.color = true

Pry.config.prompt = proc do |obj, nest_level, _pry_|
  version = ''
  version << "\001\e[0;31m\002"
  version << "Rails#{Rails.version}/" if defined? Rails
  version << "Ruby#{RUBY_VERSION}"
  version << "\001\e[0m\002"

  "#{version} #{Pry.config.prompt_name}(#{Pry.view_clip(obj)})> "
end
