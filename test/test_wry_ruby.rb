require "test_helper"

class TestWryRuby < Minitest::Test
  # def test_that_it_has_a_version_number
  #   refute_nil ::WryRuby::VERSION
  # end

  def test_hello_wry
    # title, width, height, resizable , timeout (seconds) to close window during test
    new_window "Hello Wry", 100, 400, true, 3
  end
end
