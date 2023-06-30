require 'test_helper'

class TestWryRuby < Minitest::Test
  # def test_that_it_has_a_version_number
  #   refute_nil ::WryRuby::VERSION
  # end

  def test_hello_wry
    new_window "Hello Wry",100,400,true # title, width, height, resizable
  end
end
