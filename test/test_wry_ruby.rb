require "test_helper"
require "timeout"

class TestWryRuby < Minitest::Test
  def test_hello_wry
    timeout_seconds = 1
    assert_timeout(timeout_seconds) do
      new_window "Hello Wry", 100, 400, true, 1 # title, width, height, resizable
    end
  end

  private

  def assert_timeout(timeout_seconds, &block)
    Timeout.timeout(timeout_seconds, &block)
  rescue Timeout::Error
    raise "Window creation timed out after #{timeout_seconds}s."
  end
end
