# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/testtask"

Rake::TestTask.new(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/**/test_*.rb"]
end

require "rubocop/rake_task"

RuboCop::RakeTask.new

require "rb_sys/extensiontask"

task build: :compile

RbSys::ExtensionTask.new("wry_ruby") do |ext|
  ext.lib_dir = "lib/wry_ruby"
end

task default: %i[compile test] # no rubocop add later
