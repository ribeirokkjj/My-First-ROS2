import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Twist
from std_msgs.msg import String
import ntcore

class WPILibBridgeNode(Node):
    def __init__(self):
        super().__init__('wpilib_bridge_node')

        # 1. Configuração de IP/Time (Substitua '0' pelo número do seu time FRC)
        self.declare_parameter('team_number', 0)
        self.declare_parameter('server_ip', '10.96.11.2')  # Use 10.TE.AM.2 para roboRIO real ou 127.0.0.1 para simulação

        team_number = self.get_parameter('team_number').get_parameter_value().integer_value
        server_ip = self.get_parameter('server_ip').get_parameter_value().string_value

        # 2. Conectar à NetworkTable
        self.nt_instance = ntcore.NetworkTableInstance.getDefault()
        self.nt_instance.startClient4("ROS2_Node")

        if team_number > 0:
            self.nt_instance.setServerTeam(team_number)
            self.get_logger().info(f'Conectando à roboRIO do time {team_number}...')
        else:
            self.nt_instance.setServer(server_ip)
            self.get_logger().info(f'Conectando ao Servidor NT no IP: {server_ip}...')

        # Accessa a tabela "ROS2Data" (mesmo nome usado no Java!)
        self.table = self.nt_instance.getTable("ROS2Data")

        # 3. Publishers para a NetworkTable (Igual aos tópicos que o Java espera)
        self.nt_linear_x = self.table.getFloatTopic("linear_x").publish()
        self.nt_modo_autonomo = self.table.getStringTopic("modo_autonomo").publish()

        # 4. Subscriber do Teleop no ROS 2 (Ouve o /cmd_vel)
        self.create_subscription(
            Twist,
            '/cmd_vel',
            self.teleop_callback,
            10
        )

        # 5. Subscriber para o Modo Autônomo (Opcional)
        self.create_subscription(
            String,
            '/modo_autonomo',
            self.modo_callback,
            10
        )

    def teleop_callback(self, msg: Twist):
        """Recebe o comando do teleop (teclado/joystick) e envia a velocidade para a roboRIO"""
        velocidade_x = msg.linear.x
        self.get_logger().info(f'Enviando velocidade: {velocidade_x}')
        
        # Grava no NetworkTables na chave "linear_x"
        self.nt_linear_x.set(velocidade_x)

    def modo_callback(self, msg: String):
        """Envia o estado/modo para a chave 'modo_autonomo'"""
        self.get_logger().info(f'Enviando modo: {msg.data}')
        self.nt_modo_autonomo.set(msg.data)


def main(args=None):
    rclpy.init(args=args)
    node = WPILibBridgeNode()
    
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()

if __name__ == '__main__':
    main()