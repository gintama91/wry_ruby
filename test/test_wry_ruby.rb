# frozen_string_literal: true

require "test_helper"

class TestWryRuby < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::WryRuby::VERSION
  end

  def test_hello_wry_hm
    hello_wry
  end
end
