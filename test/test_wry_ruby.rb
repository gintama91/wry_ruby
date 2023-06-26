require 'test_helper'

class TestWryRuby < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::WryRuby::VERSION
  end

  def test_hello_wry
    # assert_output("Wry has started!\n") do
    #   hello_wry
    # end
  end
end
