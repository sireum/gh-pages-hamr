// This file will not be overwritten if codegen is rerun

//----------------------------------------------------------------
//  REDUCED TEST SUITE used to demonstrate missing coverage in the
//  HAMR Rust Testing Manual ("Generating Coverage Reports" section).
//
//  Compared to the original tests.rs, this version retains only:
//    - test_initialization_REQ_THERM_1 (covers the initialize entry point)
//    - test_compute                    (covers timeTriggered with default
//                                       inputs that exercise only the
//                                       REQ_THERM_4 fall-through path)
//    - test_compute_GUMBOX_manual_1_a  (a single manual GUMBOX test that
//                                       again exercises only REQ_THERM_4)
//
//  All tests for REQ_THERM_2 and REQ_THERM_3, all GUMBOX tests for those
//  cases, and all property-based tests have been removed.  The resulting
//  coverage report shows the REQ_THERM_2/REQ_THERM_3 branches in
//  timeTriggered as uncovered, and the corresponding GUMBOX case
//  oracle bodies as only partially exercised.
//----------------------------------------------------------------


mod tests {
  // NOTE: need to run tests sequentially to prevent race conditions
  //       on the app and the testing apis which are static
  use serial_test::serial;

  use crate::test::util::*;
  use data::*;
  use data::Isolette_Data_Model::*;

  #[test]
  #[serial]
  fn test_initialization_REQ_THERM_1() {
    crate::thermostat_thermostat_initialize();

    let heat_control: On_Off = test_apis::get_heat_control();
    let post_lastCmd : On_Off = test_apis::get_lastCmd();

    // REQ_THERM_1: The Heat Control command shall be initially Off
    assert!(heat_control == On_Off::Off);
    assert!(post_lastCmd == On_Off::Off);
  }

  // auto-generated example test for `compute` entry point.
  // Default-valued inputs (Temp::default() = degrees 0; Set_Points::default()
  // = lower 0 / upper 0) leave currentTemp.degrees neither greater than the
  // upper bound nor less than the lower bound, so timeTriggered only
  // executes the REQ_THERM_4 fall-through path.
  #[test]
  #[serial]
  fn test_compute() {
    crate::thermostat_thermostat_initialize();

    test_apis::put_current_temp(Isolette_Data_Model::Temp::default());
    test_apis::put_desired_temp(Isolette_Data_Model::Set_Points::default());
    test_apis::put_lastCmd(Isolette_Data_Model::On_Off::default());

    crate::thermostat_thermostat_timeTriggered();
  }
}

mod GUMBOX_manual_tests {
  use serial_test::serial;

  use crate::test::util::*;
  use data::Isolette_Data_Model::*;

  // Single manual GUMBOX test that exercises only the REQ_THERM_4 case
  // (current_temp 98 lies inside the desired range [98, 100]).
  #[test]
  #[serial]
  fn test_compute_GUMBOX_manual_1_a() {
    crate::thermostat_thermostat_initialize();

    let current_temp = Temp { degrees: 98 };
    let lower_desired_temp = Temp { degrees: 98 };
    let upper_desired_temp = Temp { degrees: 100 };
    let desired_temp = Set_Points {lower: lower_desired_temp, upper: upper_desired_temp};
    let last_cmd = On_Off::Onn;

    let harness_result =
       cb_apis::testComputeCBwGSV(last_cmd, current_temp, desired_temp);

    assert!(matches!(harness_result, cb_apis::HarnessResult::Passed));
  }
}
