//@ HAMR: --platform JVM --output-dir . --slang-output-dir . --package-name cis771
package Isolette {

    private import HAMR::*;
    
    part def Isolette_System :> System {
        part operator_interface_process: OperatorInterface_Process;
        part control_process : Control_Process;

	    connection dt : PortConnection connect operator_interface_process.desired_temp to control_process.desired_temp;
    }

    part def OperatorInterface_Process :> Process {
        part operator_interface: OperatorInterface;

        port desired_temp : EventDataPort { out :>> type : Set_Points; }
	    connection dt : PortConnection connect operator_interface.desired_temp to desired_temp;
    }

    part def Control_Process :> Process {
        part temp_sensor: TempSensor;
        part thermostat: Thermostat;
        part heat_source: HeatSource;

        port desired_temp : EventDataPort { in :>> type : Set_Points; }

        connection tc : PortConnection connect temp_sensor.temp_changed to thermostat.temp_changed;
        connection ct : PortConnection connect temp_sensor.current_temp to thermostat.current_temp;
	    connection hc : PortConnection connect thermostat.heat_control to heat_source.heat_control;
	    connection dt : PortConnection connect desired_temp to thermostat.desired_temp;
    }

    part def TempSensor :> Thread {
        port current_temp : DataPort { out :>> type : Temp; }
        out port temp_changed : EventPort;

        attribute :>> Period = 1000 [ms];
        attribute :>> Dispatch_Protocol = Periodic;
    }

    part def PeriodicThread1s :> Thread {
        attribute :>> Period = 1 [s];
        attribute :>> Dispatch_Protocol = Periodic;
    }

    part def OperatorInterface :> PeriodicThread1s {
        port desired_temp : EventDataPort { out :>> type : Set_Points; }
    }

    part def SporadicThread :> Thread {
        attribute :>> Dispatch_Protocol = Sporadic;
    }

    part def HeatSource :> SporadicThread {
        port heat_control : EventDataPort { in :>> type : On_Off; }
    }

    part def Thermostat :> SporadicThread {
        port current_temp : DataPort { in :>> type : Temp; }
        in port temp_changed : EventPort;
        port desired_temp : EventDataPort { in :>> type : Set_Points; }
        port heat_control : EventDataPort { out :>> type : On_Off; }
    }

    part def Temp :> Struct {
        degrees : Base_Types::Integer;
    }

    part def Set_Points :> Struct {
        lower : Temp;
        upper : Temp;
    }

    enum def On_Off {
        enum On;
        enum Off;
    }

}